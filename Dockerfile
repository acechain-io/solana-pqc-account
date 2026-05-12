# ─── Stage 1: Build website (Next.js static export) ──────────────────────────
FROM node:20-alpine AS website-builder
WORKDIR /app/website
COPY website/package.json website/package-lock.json* ./
RUN npm ci --prefer-offline
COPY website/ ./
RUN npm run build
# output: website/out/

# ─── Stage 2: Build demo app (Vite static) ────────────────────────────────────
FROM node:20-alpine AS demo-builder
RUN apk add --no-cache python3 make g++ linux-headers eudev-dev
WORKDIR /app/demo
COPY ace-account-kit/app/package.json ace-account-kit/app/yarn.lock* ./
RUN yarn install --frozen-lockfile
COPY ace-account-kit/app/ ./
RUN yarn vite build
# output: demo/dist/

# ─── Stage 3: Build API server (Rust) ─────────────────────────────────────────
FROM rust:1.87-slim AS api-builder
WORKDIR /app/api
# Cache dependencies first
COPY ace-account-kit/api/Cargo.toml ace-account-kit/api/Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && echo "pub fn dummy(){}" > src/lib.rs
RUN cargo build --release
RUN rm src/main.rs src/lib.rs
# Build real source
COPY ace-account-kit/api/src/ ./src/
RUN touch src/main.rs src/lib.rs && cargo build --release
# output: target/release/ace-api

# ─── Stage 4: Final image ─────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    nginx \
    supervisor \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# API binary
COPY --from=api-builder /app/api/target/release/ace-api /usr/local/bin/ace-api

# Website static files → /var/www/website
COPY --from=website-builder /app/website/out /var/www/website

# Demo app static files → /var/www/demo
COPY --from=demo-builder /app/demo/dist /var/www/demo

# nginx config
COPY deploy/nginx.conf /etc/nginx/sites-available/default

# supervisord config
COPY deploy/supervisord.conf /etc/supervisor/conf.d/solaa.conf

EXPOSE 80

CMD ["/usr/bin/supervisord", "-n", "-c", "/etc/supervisor/supervisord.conf"]

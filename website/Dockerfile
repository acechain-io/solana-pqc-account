FROM node:20-alpine AS builder
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci --prefer-offline
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/out /usr/share/nginx/html
RUN printf 'server {\n    listen 8080;\n    root /usr/share/nginx/html;\n    location / {\n        try_files $uri $uri/ $uri.html /index.html;\n    }\n}\n' > /etc/nginx/conf.d/default.conf
EXPOSE 8080

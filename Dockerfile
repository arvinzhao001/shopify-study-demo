FROM node:20-alpine

RUN apk add --no-cache openssl

EXPOSE 3000

WORKDIR /app
COPY package*.json ./

# Install dependencies including dev dependencies for the build process
RUN npm install

# Copy the rest of the application code
COPY . .

# Set environment variable
ENV NODE_ENV=production
ENV SHOPIFY_APP_URL=https://47.76.174.219
ENV SHOPIFY_API_KEY=1d0045571748bef7d47d1af82f1e7163
ENV SHOPIFY_API_SECRET=1378fcdc3e24463db471326f974b5691

# Build the application
RUN npm run build

# Remove dev dependencies after the build
RUN npm prune --production

# Uncomment the following line if you want to remove the development SQLite database file
# RUN rm -f prisma/dev.sqlite

CMD ["npm", "run", "docker-start"]
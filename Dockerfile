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

# Build the application
RUN npm run build

# Remove dev dependencies after the build
RUN npm prune --production

# Uncomment the following line if you want to remove the development SQLite database file
# RUN rm -f prisma/dev.sqlite

CMD ["npm", "run", "docker-start"]
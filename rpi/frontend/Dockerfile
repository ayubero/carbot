# Use a multi-platform Node.js base image
FROM --platform=$BUILDPLATFORM node:24-alpine AS builder

WORKDIR /app

# Copy package.json and install dependencies
COPY package*.json ./
RUN npm install

# Copy the rest of the project files
COPY . .

# Build for production
RUN npm run build

# Use a minimal runtime image
FROM node:24-alpine
WORKDIR /app

# Copy only the built output from the builder stage
COPY --from=builder /app/.output ./.output

# Expose Nuxt port
EXPOSE 3000

# Start Nuxt in production mode
CMD ["node", ".output/server/index.mjs"]

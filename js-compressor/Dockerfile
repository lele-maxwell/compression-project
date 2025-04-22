# Use Node.js LTS version
FROM node:20-alpine

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install && npm install -g mocha

# Copy source code
COPY . .

# Run tests
RUN npm test

# Set the entrypoint to expose the CLI interface
ENTRYPOINT ["node", "index.js"]

# Default command (can be overridden)
CMD ["--help"]

# Use Node.js LTS version
FROM node:18-slim

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy source code
COPY . .

# Set the entrypoint to expose the CLI interface
ENTRYPOINT ["node", "index.js"]

# Default command (can be overridden)
CMD ["--help"]

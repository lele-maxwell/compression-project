# Use Node.js LTS version
FROM node:20-alpine

# Set working directory
WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy the rest of the source code
COPY . .

# Set the entrypoint to expose the CLI interface
ENTRYPOINT ["node", "index.js"]

# Default command (can be overridden)
CMD ["--help"]

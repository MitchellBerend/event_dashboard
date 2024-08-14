FROM node:latest AS builder

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm install

COPY . .

# RUN npm run build

# Stage 2: Serve the application
# FROM node:latest

# RUN npm install -g serve

# WORKDIR /app
#
# COPY --from=builder /app/dist .

EXPOSE 5173

# CMD ["serve", "-s", ".", "-l", "5173"]
CMD ["npm", "run", "dev", "--", "--host"]

FROM node:16.8.0

RUN mkdir -p /usr/src/app

ENV PORT 3000

WORKDIR /usr/src/app

COPY package.json /usr/src/app
# COPY yarn.lock /usr/src/app

RUN yarn

COPY . /usr/src/app

## export NODE_OPTIONS=--openssl-legacy-provider && 
RUN yarn build

EXPOSE 3000

ENTRYPOINT ["yarn", "dev"]

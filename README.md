# NestJS Starter

Opinionated NestJS MVC boilerplate for rapid development with battle-tested standards.

## Stack

This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

First, install project dependencies:

```bash
yarn 
```

Then, run the development server:

```bash
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `pages/index.js`. The page auto-updates as you edit the file.

[API routes](https://nextjs.org/docs/api-routes/introduction) can be accessed on [http://localhost:3000/api/hello](http://localhost:3000/api/hello). This endpoint can be edited in `pages/api/hello.js`.

The `pages/api` directory is mapped to `/api/*`. Files in this directory are treated as [API routes](https://nextjs.org/docs/api-routes/introduction) instead of React pages.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.


## Usage

```sh
cp .env.example .env
docker-compose up
docker-compose exec web yarn lint
docker-compose exec db psql -U postgres -c 'create database test;'
docker-compose exec web yarn test
docker-compose exec web yarn test:e2e
docker-compose exec web yarn build
```

## Functionality

REST endpoint via Nest
- http://localhost:3000/

JWT-protected REST endpoint via Nest
- http://localhost:3000/private

GraphQL playground (`query WhoAmI` is JWT-protected)
- http://localhost:3000/graphql
```qgl
query Public {
  things {
    id
    name
  }

  users {
    id
    provider
  }
}

# Add Header: { "Authorization": "Bearer <token>" }
query Private {
  whoAmI {
    id,
    provider,
    providerId,
    username,
    name
  }
  
  orders {
    id
    
    alias
    thing {
      name
    }
  }
}

mutation createOrder {
  createOrder(alias: "myname", thingName: "this is a thing you can order") {
    id
    alias
  }
}
```

Cognito auth (redirects to hosted Cognito UI)
- http://localhost:3000/auth/cognito

Google auth
- http://localhost:3000/auth/google

Next.js page
- http://localhost:3000/home

JWT-protected Next.js page
- http://localhost:3000/profile

## Deloyment

```sh
heroku git:remote --app <app-name>
heroku stack:set container
cp .env .env.production # Fill production values
xargs -a .env.production -I {} heroku config:set {}
heroku addons:create heroku-postgresql:hobby-dev
```

### Useful commands

Nest CLI:
```
docker-compose exec web yarn nest -- --help
```

TypeORM CLI:
```
docker-compose exec web yarn typeorm -- --help
```

## Resources

- https://github.com/jmcdo29/testing-nestjs

# Boot this app on Ali Cloud
1. Push code change to git, then pull code to cloud server.
2. Build docker image: docker build -t shopify-discount-study . 
3. Start shopify app: docker run -p 3000:3000 shopify-discount-study
4. Start reverse proxy ngnix: sudo service nginx restart
5. Maybe need uninstall app from shopify admin and then re-install the app

# Build docker image & run docker, can check the Dockerfile
1. Build the Docker Image: Open a terminal in the root directory of your project (where the Dockerfile is located) and run the following command to build the Docker image, This command will create a Docker image named my-node-app using the instructions in the Dockerfile.
> docker build -t my-node-app .

2. Run the Docker Container: After the image is built, you can run a container from the image with the following command,This command will start a container from the my-node-app image and map port 3000 of the container to port 3000 on your host machine.
> docker run -p 3000:3000 my-node-app

3. Access the Application: Open a web browser and navigate to http://localhost:3000 to access your Node.js application running inside the Docker container.

# Shopify app tutorial with admin extensions

This app is a guide for adding extensions to a Shopify app.

Rather than cloning this repo, you can use your preferred package manager and the Shopify CLI with [these steps](https://shopify.dev/docs/apps/getting-started/create) to create your own app and these [tutorials](https://shopify.dev/docs/apps/admin/admin-actions-and-blocks) to get started with admin action and block extensions.

This repo tracks the three tutorials that cover:

- [Building a discount function](https://shopify.dev/docs/apps/build/discounts/build-discount-function)
- [Building a discounts allocator](https://shopify.dev/docs/apps/build/discounts/build-discounts-allocator)
- [Building a discounts app with Remix](https://shopify.dev/docs/apps/build/discounts/build-ui-remix)

## Aligning this app to the tutorial

Running this app with no changes, will start you at the finishing point of [Building a discounts app with Remix](https://shopify.dev/docs/apps/build/discounts/build-ui-remix). This is the point where we have created an discount function and added a Remix route to render UI for discount creation in Shopify Admin.

## A note about additional extensions

This app is a guide for adding discount function extensions to a Shopify app. We have also provided some additional extensions to help you get started with other types of extensions. These are:

- Order discount function with minimum subtotal ([Rust extension](extensions/order-discount-rust/README.md), [Javascript extension](extensions/order-discount/README.md))

## A note on the comments

You will find magic comments, (eg. `# [START function-configuration.start]`) throughout the files in this app. These are for highlighting code in shopify.dev and can be ignored.

## Quick start

### Prerequisites

1. You must [download and install Node.js](https://nodejs.org/en/download/) if you don't already have it.
2. You must [create a Shopify partner account](https://partners.shopify.com/signup) if you don’t have one.
3. You must create a store for testing if you don't have one, either a [development store](https://help.shopify.com/en/partners/dashboard/development-stores#create-a-development-store) or a [Shopify Plus sandbox store](https://help.shopify.com/en/partners/dashboard/managing-stores/plus-sandbox-store).

### Setup

If you used the CLI to create the template, you can skip this section.

Using yarn:

```shell
yarn install
```

Using npm:

```shell
npm install
```

Using pnpm:

```shell
pnpm install
```

### Local Development

Using yarn:

```shell
yarn dev
```

Using npm:

```shell
npm run dev
```

Using pnpm:

```shell
pnpm run dev
```

Press P to open the URL to your app. Once you click install, you can start development.

Local development is powered by [the Shopify CLI](https://shopify.dev/docs/apps/tools/cli). It logs into your partners account, connects to an app, provides environment variables, updates remote config, creates a tunnel and provides commands to generate extensions.

## More about the app

For more information about the base Remix app without extensions, check out this [repo](https://github.com/Shopify/shopify-app-template-remix).

## Resources

- [Remix Docs](https://remix.run/docs/en/v1)
- [Shopify App Remix](https://shopify.dev/docs/api/shopify-app-remix)
- [Introduction to Shopify apps](https://shopify.dev/docs/apps/getting-started)
- [App authentication](https://shopify.dev/docs/apps/auth)
- [Shopify CLI](https://shopify.dev/docs/apps/tools/cli)
- [App extensions](https://shopify.dev/docs/apps/app-extensions/list)
- [Shopify Functions](https://shopify.dev/docs/api/functions)
- [Getting started with internationalizing your app](https://shopify.dev/docs/apps/best-practices/internationalization/getting-started)

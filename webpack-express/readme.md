# Running webpack-dev-server on Express

I found this short tutorial by Tom Misawa and gave it a whirl.

[You can do the same here!](https://dev.to/riversun/how-to-run-webpack-dev-server-on-express-5ei9)

This tutorial also builds a simple app that adds two numbers together. Perhaps inspired by the same tutorial I used in the other webpack tutorial I did?

## To Run

This tutorial didn't bother setting up a pipeline to create a production build and I didn't either. It won't be difficult to add later though if I feel like it. As such, you'll need to just run it in dev mode, which was what the point of the tutorial was meant to teach anyway. Just run `npm install` and `npm start` from the webpack-express directory and got to localhost:8080 in your browser to use the app.

## My Thoughts

Getting express to run my webpack-dev-server was proving to be more complicated than I imagined it would be. Looking at it here, I'm still not entirely sure I get what's going on. There's a lot of extra middleware needed to make this happen. And, perhaps I'm not getting something here, but is this tutorial actually using dev-server? I think the dev middleware and hot middleware are actually doing the work and that the dev server could be left out completely.

I might see if there is another tutorial out there on the subject to make sure this is the best way to do this.

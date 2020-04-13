# Webpack Tutorial

This is a tutorial comes from Colt at FreeCodeCamp's YouTube channel.

[You can check out his YouTube tutorial here!](https://www.youtube.com/watch?v=MpGLUVbqoYQ)

This tutorial creates a really simple application that just adds two numbers together. Complexity of the final product wasn't the purpose as much as teaching the complexities of Webpack.

## To Run

To use this for yourself, just go into the webpack directory, run `npm install`. The fastest way to get up and running is to just run `npm start` which will run the webpack dev server. If you do this, you can view the app in your web browser at localhost:8080. Otherwise, you can run `npm run build` to create a production build of the application. Once you've done this, you can just open the index.html file in the dist folder that gets created to view the application.

## My Thoughts

Create React App is a wonderful CLI to get React apps up and running quickly. Sadly, it abstracts the webpack stuff out of the application for the developer and as a result, a lot of React devs don't mess with it and few tutorials ever go over it. Webpack is fantastic, but it's not the easiest thing to learn and after letting CRA do the heavy lifting for me for so long, I decided it was time to dive back into webpack.

This was a great tutorial as it covers two of my biggest concerns, compiling multiple JS files and using it to compile SASS files into one, compact, CSS file for production. Next thing to figure out, how to get this thing to create EJS files for me.

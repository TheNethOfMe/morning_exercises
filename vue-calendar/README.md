# vue-calendar with Google Firebase

Another tutorial from Brad Traversy which seems to consist primarily of code from Vuetify.

[You can check out his YouTube tutorial here!](https://www.youtube.com/watch?v=2NOsjTT1b_k)
[Here is the Vuetify Calendar component on which this project is based.](https://vuetifyjs.com/en/components/calendars)

![Screenshot of Calendar](./screenshot.png, "Screenshot of Calendar");

## To Run

This app won't work as-is. It requires a connection to a Google Firebase collection which I'm not including in the repo for obvious reasons. You'll want to create a file called `fbdata.js` in the `src` directory and export an object with your Google Firebase connection data to make the `firebase.initializeApp` function in main.js work.

Other than that, you'll want to do all the usual stuff. I used Yarn instead of npm for package management here, so you'll want to use `yarn install` to get everything working.

## My Thoughts

Way back when I started working on my first version of My 90s Notebook, I built it with the help of a full-stack tutorial that used Firebase. I haven't really used it since, but maybe it's not a bad idea.

I've also wanted a reason to use Vue more and make something more complex with it. Perhaps my long-negletced 90s Notebook could use yet another new version? Perhaps with Vue, Vuetify and going back to Firebase?

That will require a lot of time, as I never really finished the last batch of changes I wanted to do to the site (mostly implimenting React hooks and eliminating the need for Redux) so a complete rebuild from scratch doesn't seem like an awful idea at this point.

My reasons for not wanting to use Firebase are mostly political. Google is one of the big tech giants and big tech giants are ruining our lives. But luckily I can always give myself the out that "there's not ethical consumption under capitalism" so nothing I do can truely fix anything anyway. It's not like I'm gonna give them money for it, anyway.

At least they aren't Amazon.

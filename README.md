## Estimation Engine

Simple app to facilitate planning poker over :sparkles: the internet :sparkles:  

### Design

The application is in two parts:

The UX is a bog-standard react/redux javascript application using bulma for the base styles. The only bit of trickery is using a websocket connection from the backend to handle all updates from the backend.  

The backend is a poem.rs application, being run and deployed using shuttle.rs. The data for the app is stored in a hashmap protected by an Arc/Mutex pair, and stored in a static variable.  The only other state-tracking is via a browser cookie to identify the client.  

### Developer Setup

Make sure you have access to both Cargo and Yarn.

Setup the repo locally:

    $ git clone git@github.com:atarola/estimation-engine.git
    $ cd estimation-engine
    $ cargo build
    $ yarn install

To run in dev mode, use two tabs:

    $ cargo watch --no-vcs-ignores -i ux/ -x "shuttle run"
    $ yarn watch 'yarn build' ux

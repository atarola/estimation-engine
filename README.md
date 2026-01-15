## Estimation Engine

Simple app to facilitate planning poker over :sparkles: the internet :sparkles:  

### Design

The application is in two parts:

The UX is a bog-standard react/redux javascript application using bulma for the base styles. The only bit of trickery is using a websocket connection from the backend to handle all updates from the backend.  

The backend is a poem.rs application, using a set of actors per connection, plus one for each room. 

### Developer Setup

Make sure you have access to both Cargo and Yarn.

Setup the repo locally:

    $ git clone git@github.com:atarola/estimation-engine.git
    $ cd estimation-engine
    $ cargo build
    $ yarn install

To run in dev mode, use two tabs:

    $ cargo watch --no-vcs-ignores -i ux/ -x run
    $ yarn watch 'yarn build' ux

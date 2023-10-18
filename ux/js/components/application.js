import { useSelector } from 'react-redux'

import { Header } from './header';
import { Sidebar } from './sidebar';
import { Register } from './register';
import { Voting } from './voting';
import { Results } from './results';
import { getUuid } from '../util';

export function Application() {
    let store = useSelector((store) => store);

    let content = isRegistered(store) ? 
        <MainArea /> : 
        <Register />;

    return (
        <>
            <Header />
            { content }
        </>
    );
}

function MainArea() {
    let state = useSelector((store) => store.state)

    let content = (state == "vote") ?
        <Voting />:
        <Results />;

    return (
        <section class="section main-area">
            <div class="container">
                <div class="columns">
                    <div class="column is-3">
                        <Sidebar />
                    </div>
                    <div class="column is-9">
                        <div class="content is-medium">
                            { content }
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

function isRegistered(store) {
    debugger
    let registered = true;
    registered = (registered && store.id != null);
    registered = registered && store.participants.hasOwnProperty(getUuid());
    return registered;
}
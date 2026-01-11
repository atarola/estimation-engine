import { useSelector, useDispatch } from 'react-redux';

import { resetVotes } from '../actions/reset_votes';
import { revealVotes } from '../actions/reveal_votes';

export function Sidebar() {
    return (
        <Wrapper>
            <Participants />
            <ActionButton />
        </Wrapper>
    );
}

function Wrapper({ children }) {
    return (
        <nav className="menu">
            { children }
        </nav>
    );
}

function Participants() {
    let store = useSelector((store) => store);

    // translate the votes into a list
    let voters = store.votes.map((vote) => vote.uuid);

    // tranform the participants into list items
    let participants = Object.values(store.participants).map((participant) => {
        let voted = voters.includes(participant.uuid) ? <VotedTag /> : <PendingTag />;

        return (
            <li key={participant.uuid} className="tile mb-1">
                <div className="is-flex-grow-1">{ participant.name }</div>
                <div>{ voted }</div>
            </li>
        );
    });

    return (
        <>
            <p className="menu-label">
                Participants
            </p>
            <ul className="menu-list">
                { participants }
            </ul>
        </>
    )
}

function ActionButton() {
    let state = useSelector((state) => state.state);
    let dispatch = useDispatch();

    let handler = () => {
        state == "vote" ? dispatch(revealVotes()) : dispatch(resetVotes());
    };

    return (
        <>
            <p className="menu-label">
                Actions
            </p>
            <ul className="menu-list">
                <li>
                    <button onClick={ handler } className="button is-fullwidth is-primary is-rounded">
                        { state == "vote" ? "Reveal Votes" : "Reset Votes" }
                    </button>  
                </li>
            </ul>
        </>
    );
}

function PendingTag() {
    return (
        <span className="tag icon-text">
            <span className="icon">
                <i className="fa fa-cog fa-spin"></i>
            </span>
            <span>Pending</span>
        </span>
    );
}

function VotedTag() {
    return (
        <span className="tag is-success icon-text">
            <span className="icon">
                <i className="fa fa-check-square"></i>
            </span>
            <span>Voted</span>
        </span>
    );
}

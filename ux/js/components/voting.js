import { useSelector, useDispatch } from 'react-redux';

import { getUuid } from '../util';
import { vote } from '../actions/vote';

const voteOptions = [
    {value: "xsmall", display: "XS" },
    {value: "small", display: "S" },
    {value: "medium", display: "M" },
    {value: "large", display: "L" },
    {value: "xlarge", display: "XL" }
];

const secondaryOptions = [
    {value: "coffee", display: "☕️" },
    {value: "shrug", display: "?" }
];

const emptyVote = { vote: "" };

export function Voting() {
    let votes = useSelector((store) => store.votes);

    let userUuid = getUuid();
    let userVote = (votes.find((item) => item.uuid == userUuid) || emptyVote).vote;

    let voteCards = voteOptions.map((option) => Card({...option, userVote }));
    let secondaryCards = secondaryOptions.map((option) => Card({...option, userVote }));

    return (
        <div class="box">
            <h4 class="title is-4">Vote</h4>
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-parent">
                    { voteCards }
                </div>
                <div class="tile is-parent">
                    { secondaryCards }
                </div>
            </div>
        </div>
    )
}

function Card(props) {
    let dispatch = useDispatch();

    let onClick = (event) => {
        event.preventDefault();
        dispatch(vote({size: props.value}));
    };

    let selected = (props.value == props.userVote) ?
        "is-selected" :
        "";

    return (
        <div class={`tile playing-card ${selected}`} onClick={ onClick }>
            <div class="playing-card-start is-size-7">
                { props.display }
            </div>
            <div class="playing-card-middle is-size-2">
                <span>
                    { props.display }
                </span>
            </div>
            <div class="playing-card-end is-size-7">
                { props.display }
            </div>
        </div>
    )
}


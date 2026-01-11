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

    let voteCards = voteOptions.map((option) => <Card key={option.value} {...option} userVote={userVote} />);
    let secondaryCards = secondaryOptions.map((option) => <Card key={option.value} {...option} userVote={userVote} />);

    return (
        <div className="box">
            <h4 className="title is-4">Vote</h4>
            <div className="tile is-ancestor is-vertical">
                <div className="tile is-parent">
                    { voteCards }
                </div>
                <div className="tile is-parent">
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
        <div className={`tile playing-card ${selected}`} onClick={ onClick }>
            <div className="playing-card-start is-size-7">
                { props.display }
            </div>
            <div className="playing-card-middle is-size-2">
                <span>
                    { props.display }
                </span>
            </div>
            <div className="playing-card-end is-size-7">
                { props.display }
            </div>
        </div>
    )
}

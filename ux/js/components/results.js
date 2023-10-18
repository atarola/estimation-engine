import { useSelector } from 'react-redux';

const voteOptions = {
    xsmall: "XS",
    small: "S",
    medium: "M",
    large: "L",
    xlarge: "XL",
    coffee: "☕️",
    shrug: "?"
};

export function Results() {
    let rows;
    let store = useSelector((store) => store);

    if (store.votes.length > 0) {
        rows = store.votes.map((vote) => {
            return (
                <tr>
                    <td>{ store.participants[vote.uuid].name }</td>
                    <td>{ voteOptions[vote.vote] }</td>
                </tr>
            );
        });
    } else {
        rows = (
            <tr>
                <td colSpan="2">No Votes</td>
            </tr>
        );
    }


    return (
        <div class="box">
            <h4 class="title is-4">Results</h4>
            <table class="table is-striped">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Vote</th>
                    </tr>
                </thead>
                <tbody>
                    { rows }
                </tbody>
            </table>
        </div>
    )
}
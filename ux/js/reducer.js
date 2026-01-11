import { createReducer } from '@reduxjs/toolkit';

import { getTopicId } from "./util";
import { register } from './actions/register';
import { resetVotes } from './actions/reset_votes';
import { revealVotes } from './actions/reveal_votes';
import { vote } from "./actions/vote";

const initialState = { 
    loading: true,
    error: null,
    id: getTopicId(),
    participants: {},
    state: "vote",
    votes: []
}

// the global reducer, looks at the incoming events and updates the store appropriately
export const reducer = createReducer(initialState, (builder) => {
    builder.addCase(register.pending, (state, action) => {
        return { ...state, loading: true, error: null};
    });

    builder.addCase(register.fulfilled, (state, action) => {
        return {
            ...action.payload,
            loading: false, 
            error: null
        };
    });

    builder.addCase(register.rejected, (state, action) => {
        return { ...state, loading: false, error: action.payload};
    });

    builder.addCase("room_details", (state, action) => {
        return {
            ...action.payload,
            loading: false, 
            error: null
        }
    });
});
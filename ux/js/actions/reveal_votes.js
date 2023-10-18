import { createAsyncThunk } from '@reduxjs/toolkit';

import { getTopicId } from '../util';

// reveal votes
export const revealVotes = createAsyncThunk(
    "revealVotes", 
    async () => {
        let options = {
            method: "POST",
            mode: "same-origin",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({})
        };
        
        return await fetch(`/api/${getTopicId()}/reveal`, options)
            .then((response) => response.body);
    }
);

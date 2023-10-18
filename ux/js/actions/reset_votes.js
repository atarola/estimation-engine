import { createAsyncThunk } from '@reduxjs/toolkit';

import { getTopicId } from '../util';

// resetVotes
export const resetVotes = createAsyncThunk(
    "resetVotes", 
    async () => {
        let options = {
            method: "POST",
            mode: "same-origin",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({})
        };
        
        return await fetch(`/api/${getTopicId()}/reset`, options)
            .then((response) => response.body);
    }
);
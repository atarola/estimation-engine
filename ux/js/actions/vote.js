import { createAsyncThunk } from '@reduxjs/toolkit';

import { getTopicId } from '../util';

// register with the backend
export const vote = createAsyncThunk(
    "vote", 
    async (userData) => {
        const { size } = userData;

        let options = {
            method: "POST",
            mode: "same-origin",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                vote: size
            })
        };
        
        return await fetch(`/api/${getTopicId()}/vote`, options)
            .then((response) => response.body);
    }
);

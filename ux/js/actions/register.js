import { createAsyncThunk } from '@reduxjs/toolkit';

import { createWebSocket } from '../socket';
import { getTopicId } from '../util';

// register with the backend
export const register = createAsyncThunk(
    "register", 
    async (userData, thunkApi) => {
        const { name } = userData;

        let options = {
            method: "POST",
            mode: "same-origin",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                name: name, 
                id: getTopicId()
            })
        };
        
        return await fetch("/api/register", options)
            .then((response) => response.json())
            .then((value) => {
                history.pushState(null, "", `/topic/${value.id}`);
                createWebSocket(value.id, name, thunkApi.dispatch);
                return Promise.resolve(value);
            });
    }
);

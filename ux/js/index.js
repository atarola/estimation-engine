import { createRoot } from "react-dom/client";
import { Provider } from 'react-redux'

import { Application } from "./components/application";
import { store } from './store';

window.store = store;

createRoot(document.getElementById("content")).render(
    <Provider store={store}>
        <Application />
    </Provider>
);

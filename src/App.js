import React, { useContext, useEffect } from 'react';
import { appStore, onAppMount } from './state/app';

// helpers
export const btnClass = 'btn btn-sm btn-outline-primary mb-3 '
export const flexClass = 'd-flex justify-content-evenly align-items-center '
export const qs = (s) => document.querySelector(s)

const App = () => {
    const { state, dispatch, update } = useContext(appStore);

    const onMount = () => {
        dispatch(onAppMount());
    };
    useEffect(onMount, []);

    console.log('app_state', state);
}

export default App;

import { useState } from 'react';
import { useDispatch } from 'react-redux';

import { register } from '../actions/register';

export function Register() {
    const [name, setName] = useState('');
    const dispatch = useDispatch();

    // stash the name change
    function onNameChange(e) {
        setName(e.target.value);
    }

    // handle form submits
    function onFormSubmit(e) {
        e.preventDefault();
        return dispatch(register({ name: name }));
    }

    return (
        <section class="hero is-fullheight-with-navbar main-area">
            <div class="hero-body">
                <div class="container has-text-centered">
                    <div class="column is-4 is-offset-4">
                        <h3 class="title has-text-black">Register</h3>
                        <div class="box">
                            <form onSubmit={ onFormSubmit }>
                                <div class="field">
                                    <div class="control">
                                        <input class="input" 
                                               type="text" 
                                               placeholder="Your Name" 
                                               autofocus=""
                                               onChange={ onNameChange }
                                               value={ name } />
                                    </div>
                                </div>

                                <input type="submit" 
                                       class="button is-block is-info is-fullwidth"
                                       value="Login" />
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    )
}
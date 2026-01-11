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
        <section className="hero is-fullheight-with-navbar main-area">
            <div className="hero-body">
                <div className="container has-text-centered">
                    <div className="column is-4 is-offset-4">
                        <h3 className="title has-text-black">Register</h3>
                        <div className="box">
                            <form onSubmit={ onFormSubmit }>
                                <div className="field">
                                    <div className="control">
                                        <input className="input" 
                                               type="text" 
                                               placeholder="Your Name" 
                                               autoFocus
                                               onChange={ onNameChange }
                                               value={ name } />
                                    </div>
                                </div>

                                <input type="submit" 
                                       className="button is-block is-info is-fullwidth"
                                       value="Login" />
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    )
}

import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  analytics: PropTypes.shape({
    app_id: PropTypes.string.isRequired,
    action_id: PropTypes.string.isRequired,
    user_name: PropTypes.string.isRequired
  })
};

export default function Form({ onSubmit, currentUser, analytics, disabled }) {
  const [formData, setFormData] = useState(analytics);

  const handleChange = (e) => {
    setFormData(data => ({...data,
      [e.target.name]: e.target.value
    }))
  }

  const handleSubmit = (e) => {
    e.preventDefault()
    onSubmit(formData)
  }

  useEffect(() => {
    setFormData(analytics)
  }, [analytics])

  return (
    <form>
      <fieldset id="fieldset" disabled={disabled}>
        <p>Sign into log analytics, {currentUser.accountId}!</p>
        
        <p>
          <label htmlFor="app_id">App #</label>
          <input
            autoComplete="off"
            autoFocus
            name="app_id"
            id="app_id"
            value={ formData.app_id }
            onChange={handleChange}
            required />
        </p>
        <p>
          <label htmlFor="action_id">Action #</label>
          <input
            autoComplete="off"
            name="action_id"
            id="action_id"
            value={ formData.action_id }
            onChange={handleChange}
            required />
        </p>
        <p>
          <label htmlFor="user_name">User name</label>
          <input
            autoComplete="off"
            name="user_name"
            id="user_name"
            value={ formData.user_name }
            onChange={handleChange}
            required />
        </p>

        <button type="submit" onClick={handleSubmit}>
          Submit
        </button>
      </fieldset>
    </form>
  );
}

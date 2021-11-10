import React, { useState, useEffect } from 'react';
import MultipleValueTextInput from 'react-multivalue-text-input';
import PropTypes from 'prop-types';

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  tags: PropTypes.shape({
    tags: PropTypes.string.isRequired
  })
};

export default function Form({ onSubmit, currentUser, tags, disabled }) {
  const [formData, setFormData] = useState(tags);

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
    setFormData(tags)
  }, [tags])

  return (
    <form>
      <fieldset id="fieldset" disabled={disabled}>
        <p>Sign into tag analytics, {currentUser.accountId}!</p>

        <p>
          <label htmlFor="account_id">Wallet</label>
          <input
            autoComplete="off"
            autoFocus
            name="account_id"
            value={ formData.accountId }
            onChange={handleChange}
            required />
        </p>
        <p>
          <MultipleValueTextInput
            onItemAdded={(item, allItems) => console.log(`Item added: ${item}`)}
            onItemDeleted={(item, allItems) => console.log(`Item removed: ${item}`)}
            label="Tags"
            name="tags"
            placeholder="Click on cloud tags to add"
          />
        </p>

        <button type="submit" onClick={handleSubmit}>
          Submit
        </button>
      </fieldset>
    </form>
  );
}

import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';

const AddCredentialPage: React.FC = () => {
  const navigate = useNavigate();
  const [serviceName, setServiceName] = useState('');
  const [clientId, setClientId] = useState('');
  const [clientSecret, setClientSecret] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!serviceName || !clientId || !clientSecret) {
      setError('All fields are required.');
      return;
    }

    try {
      await invoke('add_service_credential', {
        payload: {
          service_name: serviceName,
          client_id: clientId,
          client_secret: clientSecret,
        },
      });
      navigate('/credentials'); // Navigate back to the list on success
    } catch (err) {
      console.error("Failed to add credential:", err);
      setError(typeof err === 'string' ? err : 'An unknown error occurred');
    }
  };

  return (
    <div>
      <h1>Add New Credential</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <label htmlFor="serviceName">Service Name:</label>
          <input
            id="serviceName"
            type="text"
            value={serviceName}
            onChange={(e) => setServiceName(e.target.value)}
          />
        </div>
        <div>
          <label htmlFor="clientId">Client ID:</label>
          <input
            id="clientId"
            type="text"
            value={clientId}
            onChange={(e) => setClientId(e.target.value)}
          />
        </div>
        <div>
          <label htmlFor="clientSecret">Client Secret:</label>
          <input
            id="clientSecret"
            type="password"
            value={clientSecret}
            onChange={(e) => setClientSecret(e.target.value)}
          />
        </div>
        {error && <p style={{ color: 'red' }}>{error}</p>}
        <button type="submit">Save Credential</button>
      </form>
    </div>
  );
};

export default AddCredentialPage;

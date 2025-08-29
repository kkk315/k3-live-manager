import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';

// BackendのServiceCredential構造体と型を合わせる
interface ServiceCredential {
  id: number;
  service_name: string;
  client_id: string;
  client_secret: string;
}

const CredentialsListPage: React.FC = () => {
  const [credentials, setCredentials] = useState<ServiceCredential[]>([]);

  const fetchCredentials = async () => {
    try {
      const creds = await invoke<ServiceCredential[]>('get_service_credentials');
      setCredentials(creds);
    } catch (error) {
      console.error("Failed to fetch credentials:", error);
    }
  };

  useEffect(() => {
    fetchCredentials();
  }, []);

  const handleAuthenticate = async (credentialId: number) => {
    try {
      console.log(`Starting authentication for credential ID: ${credentialId}`);
      const authUrl = await invoke<string>('start_oauth_flow', { credentialId });
      console.log(`Received auth URL: ${authUrl}`);

      if (authUrl) {
        // TauriのopenUrl APIで外部ブラウザで開く
        await openUrl(authUrl);
        console.log("Opened auth URL in browser.");
      } else {
        throw new Error("Received an empty auth URL from the backend.");
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error("Failed to start OAuth flow:", errorMessage);
      alert(`Failed to start authentication: ${errorMessage}`);
    }
  };

  return (
    <div>
      <h1>Credentials</h1>
      <Link to="/credentials/add">
        <button>Register New Credential</button>
      </Link>
      <ul>
        {credentials.map((cred) => (
          <li key={cred.id}>
            {cred.service_name} (ID: {cred.id})
            <button onClick={() => handleAuthenticate(cred.id)}>
              Authenticate
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default CredentialsListPage;

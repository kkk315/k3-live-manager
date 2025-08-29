import React from 'react';
import { Routes, Route, Link } from 'react-router-dom';
import HomePage from './pages/HomePage';
import CredentialsListPage from './pages/CredentialsListPage';
import AddCredentialPage from './pages/AddCredentialPage';

const App: React.FC = () => {
  return (
    <div className="App">
      <nav>
        <ul>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/credentials">Credentials</Link>
          </li>
        </ul>
      </nav>

      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/credentials" element={<CredentialsListPage />} />
          <Route path="/credentials/add" element={<AddCredentialPage />} />
        </Routes>
      </main>
    </div>
  );
};

export default App;

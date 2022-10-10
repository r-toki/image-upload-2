import { ReactNode } from 'react';
import { Link } from 'react-router-dom';

export const AppLayout = ({ children }: { children: ReactNode }) => {
  return (
    <div
      style={{
        width: '720px',
        margin: '0 auto',
        padding: '16px',
        display: 'flex',
        flexDirection: 'column',
        gap: '16px',
      }}
    >
      <div style={{ display: 'flex', gap: '8px' }}>
        <Link to="/">Lading</Link>
        <Link to="/about">About</Link>
      </div>
      <div>{children}</div>
    </div>
  );
};

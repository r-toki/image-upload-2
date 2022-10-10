import { ReactNode } from 'react';
import { BrowserRouter } from 'react-router-dom';

export const AppProvider = ({ children }: { children: ReactNode }) => {
  return <BrowserRouter>{children}</BrowserRouter>;
};

import { useEffect } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { axios } from '@/lib/axios';

export const Landing = () => {
  useEffect(() => {
    axios.get('/').then(console.log);
  }, []);

  return (
    <AppLayout>
      <div>Landing</div>
    </AppLayout>
  );
};

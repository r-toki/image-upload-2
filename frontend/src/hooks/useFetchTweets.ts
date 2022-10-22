import { useAsyncFn } from 'react-use';

import { axios } from '@/lib/axios';

export type Tweet = {
  id: string;
  body: string;
  blobIds: string[];
  createdAt: string;
};

export const useFetchTweets = () => {
  return useAsyncFn(async (): Promise<Tweet[]> => {
    return axios.get('/tweets').then((res) => res.data);
  });
};

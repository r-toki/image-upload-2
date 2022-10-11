import { useAsyncFn } from 'react-use';

import { axios } from '@/lib/axios';

type CreateTweet = {
  body: string;
  blobIds: string[];
};

export const useCreateTweet = () => {
  return useAsyncFn(async ({ body, blobIds }: CreateTweet): Promise<null> => {
    return axios.post('/tweets', { body, blobIds }).then((res) => res.data);
  });
};

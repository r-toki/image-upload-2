import { useEffect } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { useCreateBlob } from '@/hooks/useCreateBlob';
import { useCreateTweet } from '@/hooks/useCreateTweet';
import { useFetchTweets } from '@/hooks/useFetchTweets';

import { TweetItem } from '../components/TweetItem';
import { TweetNewForm, TweetNewFormValues } from '../components/TweetNewForm';

export const Home = () => {
  const [state, fetchTweets] = useFetchTweets();

  const [, createBlob] = useCreateBlob();
  const [, createTweet] = useCreateTweet();

  useEffect(() => {
    fetchTweets();
  }, []);

  const onSubmitNewTweet = async ({ body, files }: TweetNewFormValues) => {
    const blobIds = await Promise.all(files.map(createBlob));
    await createTweet({ body, blobIds });
  };

  return (
    <AppLayout>
      <TweetNewForm onSubmit={onSubmitNewTweet} />
      <hr />
      <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
        {state.value?.map((tweet) => (
          <TweetItem key={tweet.id} tweet={tweet} />
        ))}
      </div>
    </AppLayout>
  );
};

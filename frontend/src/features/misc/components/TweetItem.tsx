import { useEffect } from 'react';

import { useFetchBlobs } from '@/hooks/useFetchBlobs';
import { Tweet } from '@/hooks/useFetchTweets';

import { ImageItem } from './ImageItem';

export type TweetItemProps = {
  tweet: Tweet;
};

export const TweetItem = ({ tweet }: TweetItemProps) => {
  const [state, fetchBlobs] = useFetchBlobs();

  useEffect(() => {
    fetchBlobs(tweet.blobIds);
  }, []);

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: '8px',
        padding: '16px 16px',
        border: 'solid 1px',
      }}
    >
      <div>{tweet.body}</div>
      {state.value && (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
          {state.value.map((b, i) => (
            <ImageItem key={i} src={b.src} />
          ))}
        </div>
      )}
    </div>
  );
};

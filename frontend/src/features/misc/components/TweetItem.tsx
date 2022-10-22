import { Tweet } from '@/hooks/useFetchTweets';

export type TweetItemProps = {
  tweet: Tweet;
};

export const TweetItem = ({ tweet }: TweetItemProps) => {
  return (
    <div>
      <div>{tweet.body}</div>
    </div>
  );
};

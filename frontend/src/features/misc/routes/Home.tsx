import { FormEventHandler, useEffect } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { useCreateBlob } from '@/hooks/useCreateBlob';
import { useCreateTweet } from '@/hooks/useCreateTweet';
import { useFetchTweets } from '@/hooks/useFetchTweets';
import { useMultipleFileInput } from '@/hooks/useMultipleFileInput';
import { useObjectUrl } from '@/hooks/useObjectUrl';
import { useTextInput } from '@/hooks/useTextInput';

export const Home = () => {
  const [state, fetchTweets] = useFetchTweets();

  useEffect(() => {
    fetchTweets();
  }, []);

  console.log(state);

  return (
    <AppLayout>
      <TweetCreateForm />
    </AppLayout>
  );
};

const TweetCreateForm = () => {
  const [, createBlob] = useCreateBlob();
  const [, createTweet] = useCreateTweet();

  const bodyInput = useTextInput();
  const multipleFilesInput = useMultipleFileInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();
    const body = bodyInput.value;
    const files = multipleFilesInput.value;

    const blobIds = await Promise.all(files.map(createBlob));
    await createTweet({ body, blobIds });

    bodyInput.reset();
    multipleFilesInput.reset();
  };

  return (
    <form
      onSubmit={onSubmit}
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: '8px',
        alignItems: 'start',
        width: '480px',
      }}
    >
      <textarea
        value={bodyInput.value}
        onChange={bodyInput.onChange}
        style={{ alignSelf: 'stretch' }}
        rows={5}
        required
      />

      <input
        type="file"
        accept="image/*"
        ref={multipleFilesInput.ref}
        onChange={multipleFilesInput.onChange}
        style={{ display: 'none' }}
      />
      <button type="button" onClick={multipleFilesInput.onClick}>
        Upload Image
      </button>

      {multipleFilesInput.value.length > 0 && (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
          {multipleFilesInput.value.map((file, i) => (
            <ImageFileItem key={i} file={file} onRemove={() => multipleFilesInput.remove(i)} />
          ))}
        </div>
      )}

      <button type="submit" disabled={!bodyInput.value}>
        Submit
      </button>
    </form>
  );
};

const ImageFileItem = ({ file, onRemove }: { file: File; onRemove: () => void }) => {
  const { objectUrl } = useObjectUrl(file);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '4px', alignItems: 'start' }}>
      <img
        src={objectUrl}
        width="200px"
        height="200px"
        style={{ border: '1px', borderRadius: '5%', objectFit: 'cover' }}
      />
      <button type="button" onClick={onRemove}>
        Delete
      </button>
    </div>
  );
};

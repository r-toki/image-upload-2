import { FormEventHandler } from 'react';

import { useMultipleFileInput } from '@/hooks/useMultipleFileInput';
import { useObjectUrl } from '@/hooks/useObjectUrl';
import { useTextInput } from '@/hooks/useTextInput';

import { ImageItem } from './ImageItem';

export type TweetNewFormValues = {
  body: string;
  files: File[];
};

export type TweetNewFormProps = {
  onSubmit: (v: TweetNewFormValues) => Promise<void>;
};

export const TweetNewForm = ({ onSubmit }: TweetNewFormProps) => {
  const bodyInput = useTextInput();
  const multipleFilesInput = useMultipleFileInput();

  const handleSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    const body = bodyInput.value;
    const files = multipleFilesInput.value;

    await onSubmit({ body, files });

    bodyInput.reset();
    multipleFilesInput.reset();
  };

  return (
    <form
      onSubmit={handleSubmit}
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: '8px',
        alignItems: 'start',
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
            <ImageFileItem key={i} file={file} onDelete={() => multipleFilesInput.remove(i)} />
          ))}
        </div>
      )}

      <button type="submit" disabled={!bodyInput.value}>
        Submit
      </button>
    </form>
  );
};

const ImageFileItem = ({ file, onDelete }: { file: File; onDelete: () => void }) => {
  const { objectUrl } = useObjectUrl(file);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '4px', alignItems: 'start' }}>
      <ImageItem src={objectUrl || ''} />
      <button type="button" onClick={onDelete}>
        Delete
      </button>
    </div>
  );
};

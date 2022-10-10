import { FormEventHandler } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { useCreateBlob } from '@/hooks/useCreateBlob';
import { useFileInput } from '@/hooks/useFileInput';
import { useObjectUrl } from '@/hooks/useObjectUrl';

export const NewImage = () => {
  const [, createBlob] = useCreateBlob();

  const fileInput = useFileInput();
  const { objectUrl } = useObjectUrl(fileInput.value);

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();
    if (!fileInput.value) return;
    await createBlob(fileInput.value);
    fileInput.reset();
  };

  return (
    <AppLayout>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <div>NewImage</div>
        <form
          onSubmit={onSubmit}
          style={{ display: 'flex', flexDirection: 'column', gap: '8px', alignItems: 'start' }}
        >
          <input
            type="file"
            ref={fileInput.ref}
            onChange={fileInput.onChange}
            style={{ display: 'none' }}
          />
          <div style={{ display: 'flex', gap: '8px' }}>
            <button onClick={fileInput.onClick}>Upload</button>
            <button onClick={fileInput.reset} disabled={!fileInput.value}>
              Reset
            </button>
          </div>

          {objectUrl && (
            <img
              src={objectUrl}
              width="280px"
              height="280px"
              style={{ border: '1px', borderRadius: '5%', objectFit: 'cover' }}
            />
          )}
          <button type="submit" disabled={!fileInput.value}>
            Submit
          </button>
        </form>
      </div>
    </AppLayout>
  );
};

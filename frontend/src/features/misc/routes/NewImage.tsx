import { FormEventHandler } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { useFileInput } from '@/hooks/useFileInput';
import { useObjectUrl } from '@/hooks/useObjectUrl';
import { axios } from '@/lib/axios';
import { EncodedFile } from '@/lib/encoded-file';

export const NewImage = () => {
  const fileInput = useFileInput();
  const { objectUrl } = useObjectUrl(fileInput.value);

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();
    if (fileInput.value) {
      const encodedFile = await EncodedFile.fromFile(fileInput.value);
      await axios
        .post('/blobs', {
          encodedBytes: encodedFile.encodedBytes,
          contentType: encodedFile.contentType,
        })
        .then(console.log);
      fileInput.reset();
    }
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
          <button onClick={fileInput.onClick}>Upload</button>
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

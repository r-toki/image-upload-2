import { FormEventHandler } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { useFileInput } from '@/hooks/useFileInput';
import { useObjectUrl } from '@/hooks/useObjectUrl';
import { encodeFile } from '@/lib/encode-file';

export const NewImage = () => {
  const fileInput = useFileInput();
  const { objectUrl } = useObjectUrl(fileInput.value);

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();
    if (fileInput.value) {
      const { contentType, encoded } = await encodeFile(fileInput.value);
      console.log(contentType, encoded);
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
              style={{ border: '1px', borderRadius: '10%' }}
            />
          )}
        </form>
      </div>
    </AppLayout>
  );
};

import { useAsyncFn } from 'react-use';

import { axios } from '@/lib/axios';
import { EncodedFile } from '@/lib/encoded-file';

export const useCreateBlob = () => {
  return useAsyncFn(async (file: File): Promise<string> => {
    const encodedFile = await EncodedFile.fromFile(file);
    const blobId = await axios
      .post<string>('/blobs', {
        encodedBytes: encodedFile.encodedBytes,
        metadata: {
          contentType: encodedFile.contentType,
        },
      })
      .then((res) => res.data);
    return blobId;
  });
};

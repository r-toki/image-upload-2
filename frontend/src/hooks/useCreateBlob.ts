import axios from 'axios';
import { useAsyncFn } from 'react-use';

import { EncodedFile } from '@/lib/encoded-file';

export const useCreateBlob = () => {
  return useAsyncFn(async (file: File) => {
    const encodedFile = await EncodedFile.fromFile(file);
    const blobId = await axios
      .post<string>('/blobs', {
        encodedBytes: encodedFile.encodedBytes,
        contentType: encodedFile.contentType,
      })
      .then((res) => res.data);
    return blobId;
  });
};

import { useAsyncFn } from 'react-use';

import { axios } from '@/lib/axios';
import { fromFile } from '@/lib/encode-file';

export const useCreateBlob = () => {
  return useAsyncFn(async (file: File): Promise<string> => {
    const { encodedBytes, contentType } = await fromFile(file);
    const blobId = await axios
      .post<string>('/blobs', {
        encodedBytes,
        contentType,
      })
      .then((res) => res.data);
    return blobId;
  });
};

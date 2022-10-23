import { useAsyncFn } from 'react-use';

import { axios } from '@/lib/axios';

export type BlobResponse = {
  src: string;
  createdAt: string;
};

export const useFetchBlobs = () => {
  return useAsyncFn(async (blob_ids: string[]): Promise<BlobResponse[]> => {
    return Promise.all(
      blob_ids.map((blob_id) => axios.get(`/blobs/${blob_id}`).then((res) => res.data)),
    );
  });
};

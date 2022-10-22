import { useAsyncFn } from 'react-use';

import { axios } from '@/lib/axios';

export type BlobResponse = {
  src: string;
  createdAt: string;
};

export const useFetchBlob = () => {
  return useAsyncFn(async (blob_id: string): Promise<BlobResponse> => {
    return axios.get(`/blobs/${blob_id}`).then((res) => res.data);
  });
};

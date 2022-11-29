-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public.posts (
    id SERIAL PRIMARY KEY,
    text text NOT NULL
);

CREATE TABLE IF NOT EXISTS public.post_image (
    id SERIAL PRIMARY KEY,
    path character varying NOT NULL,
    post_id integer NOT NULL
);

ALTER TABLE ONLY public.post_image
    ADD CONSTRAINT post_image_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.posts(id) ON UPDATE RESTRICT ON DELETE CASCADE;


CREATE TABLE IF NOT EXISTS public.post_video (
    id SERIAL PRIMARY KEY,
    path character varying NOT NULL,
    post_id integer NOT NULL
);

ALTER TABLE ONLY public.post_video
    ADD CONSTRAINT post_video_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.posts(id) ON UPDATE RESTRICT ON DELETE CASCADE;

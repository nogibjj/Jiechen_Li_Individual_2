```sql
INSERT INTO MusicTracks (artist_name, in_spotify_playlists, in_apple_playlists, bpm) 
VALUES ('Latto, Jung Kook', 553, 43, 125);
```

```sql
SELECT * FROM MusicTracks WHERE artist_name='Latto, Jung Kook';
```

```sql
UPDATE MusicTracks 
SET artist_name='Latto, Jung Kook', in_spotify_playlists=553, in_apple_playlists=43, bpm=125 
WHERE id=1;
```

```sql
DELETE FROM MusicTracks WHERE id=1;
```

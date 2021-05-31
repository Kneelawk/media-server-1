import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BrowseMediaFileComponent } from './browse-media-file.component';

describe('BrowseMediaFileComponent', () => {
  let component: BrowseMediaFileComponent;
  let fixture: ComponentFixture<BrowseMediaFileComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BrowseMediaFileComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BrowseMediaFileComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
